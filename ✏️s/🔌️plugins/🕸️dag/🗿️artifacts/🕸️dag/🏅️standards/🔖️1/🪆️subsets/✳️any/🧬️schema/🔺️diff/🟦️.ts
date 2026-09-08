/** 🔺️ DAG diff schema — sparse field delta. */
export interface DagDiff {
  /** @state artifact */ artifact?: DagArtifact;
  /** @state artifact */ schema?: string;
  /** @state artifact */ nodes?: DagNodesDelta;
  /** @state artifact */ edges?: DagEdgesDelta;
    /** @state artifact */ setNodes?: DagNodeSpecList;
    /** @state artifact */ setEdges?: DagFixtureEdgeList;
  /** @state presence */ selectedNodeIds?: DagStringList;
  /** @state config */ camera?: DagCamera;
}
export interface DagStringList { values: string[]; }
export interface DagNodesDelta { added: DagNodeSpec[]; removed: string[]; patched: DagNodePatchEntry[]; reordered?: string[]; }
export interface DagEdgesDelta { added: DagFixtureEdge[]; removed: string[]; patched: DagEdgePatchEntry[]; reordered?: string[]; }
export interface DagNodePatchEntry { id: string; patch: DagNodePatch; }
export interface DagEdgePatchEntry { id: string; patch: DagEdgePatch; }
export interface DagNodePatch { name?: string; x?: number; y?: number; }
export interface DagEdgePatch { source?: string; target?: string; }
export interface DagNodeSpecList { values: DagNodeSpec[]; }
export interface DagFixtureEdgeList { values: DagFixtureEdge[]; }
export interface DagNodeSpec { id: string; [key: string]: unknown; }
export interface DagFixtureEdge { id: string; source: string; target: string; }
export interface DagCamera { x: number; y: number; zoom: number; }
export interface DagArtifact {
  schema: string; nodes: DagNodeSpec[]; edges: DagFixtureEdge[];
  selectedNodeIds: string[]; camera: DagCamera;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class dagDagDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const dagDagDiffGuardReject = (at: string, why: string): never => {
  throw new dagDagDiffGuardRefusal(at, why);
};

type dagDagDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type dagDagDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type dagDagDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const dagDagDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : dagDagDiffGuardReject(at, "value is not an object");
export const dagDagDiffGuardArray = (value: unknown, at: string, bounds: dagDagDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return dagDagDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) dagDagDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) dagDagDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const dagDagDiffGuardString = (value: unknown, at: string, bounds: dagDagDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return dagDagDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) dagDagDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) dagDagDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) dagDagDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const dagDagDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : dagDagDiffGuardReject(at, "value is not a boolean"));
export const dagDagDiffGuardNumber = (value: unknown, at: string, bounds: dagDagDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return dagDagDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) dagDagDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) dagDagDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const dagDagDiffGuardInteger = (value: unknown, at: string, bounds: dagDagDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? dagDagDiffGuardNumber(value, at, bounds) : dagDagDiffGuardReject(at, "value is not an integer");
export const dagDagDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : dagDagDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const dagDagDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : dagDagDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDagStringList(value: unknown, at = "$"): DagStringList {
  const row = dagDagDiffGuardObject(value, at);
  return {
    values: dagDagDiffGuardArray(row["values"], `${at}.values`).map((item, index) => dagDagDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export function parseDagNodePatchEntry(value: unknown, at = "$"): DagNodePatchEntry {
  const row = dagDagDiffGuardObject(value, at);
  return {
    id: dagDagDiffGuardString(row["id"], `${at}.id`),
    patch: parseDagNodePatch(row["patch"], `${at}.patch`),
  };
}

export function parseDagEdgePatchEntry(value: unknown, at = "$"): DagEdgePatchEntry {
  const row = dagDagDiffGuardObject(value, at);
  return {
    id: dagDagDiffGuardString(row["id"], `${at}.id`),
    patch: parseDagEdgePatch(row["patch"], `${at}.patch`),
  };
}

export function parseDagNodePatch(value: unknown, at = "$"): DagNodePatch {
  return dagDagDiffGuardObject(value, `${at}`);
}

export function parseDagEdgePatch(value: unknown, at = "$"): DagEdgePatch {
  const row = dagDagDiffGuardObject(value, at);
  return {
    source: row["source"] === undefined ? undefined : dagDagDiffGuardString(row["source"], `${at}.source`),
    target: row["target"] === undefined ? undefined : dagDagDiffGuardString(row["target"], `${at}.target`),
  };
}
