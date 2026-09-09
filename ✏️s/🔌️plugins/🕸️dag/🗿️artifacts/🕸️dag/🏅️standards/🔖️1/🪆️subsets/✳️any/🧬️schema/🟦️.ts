/** 🧬️ DAG artifact schema — every field with its state class. */
export interface DagArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ nodes: DagNodeSpec[];
  /** @state artifact */ edges: DagFixtureEdge[];
}
export interface DagNodeSpec { id: string; [key: string]: unknown; }
export interface DagFixtureEdge { id: string; source: string; target: string; }
export interface DagCamera { x: number; y: number; zoom: number; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class dagDagArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const dagDagArtifactGuardReject = (at: string, why: string): never => {
  throw new dagDagArtifactGuardRefusal(at, why);
};

type dagDagArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type dagDagArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type dagDagArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const dagDagArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : dagDagArtifactGuardReject(at, "value is not an object");
export const dagDagArtifactGuardArray = (value: unknown, at: string, bounds: dagDagArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return dagDagArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) dagDagArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) dagDagArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const dagDagArtifactGuardString = (value: unknown, at: string, bounds: dagDagArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return dagDagArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) dagDagArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) dagDagArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) dagDagArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const dagDagArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : dagDagArtifactGuardReject(at, "value is not a boolean"));
export const dagDagArtifactGuardNumber = (value: unknown, at: string, bounds: dagDagArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return dagDagArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) dagDagArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) dagDagArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const dagDagArtifactGuardInteger = (value: unknown, at: string, bounds: dagDagArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? dagDagArtifactGuardNumber(value, at, bounds) : dagDagArtifactGuardReject(at, "value is not an integer");
export const dagDagArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : dagDagArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const dagDagArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : dagDagArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDagArtifact(value: unknown, at = "$"): DagArtifact {
  const row = dagDagArtifactGuardObject(value, at);
  return {
    schema: dagDagArtifactGuardString(row["schema"], `${at}.schema`),
    nodes: dagDagArtifactGuardArray(row["nodes"], `${at}.nodes`).map((item, index) => parseDagNodeSpec(item, `${at}.nodes[${index}]`)),
    edges: dagDagArtifactGuardArray(row["edges"], `${at}.edges`).map((item, index) => parseDagFixtureEdge(item, `${at}.edges[${index}]`)),
  };
}

export function parseDagNodeSpec(value: unknown, at = "$"): DagNodeSpec {
  const row = dagDagArtifactGuardObject(value, at);
  return {
    id: dagDagArtifactGuardString(row["id"], `${at}.id`),
  };
}

export function parseDagFixtureEdge(value: unknown, at = "$"): DagFixtureEdge {
  const row = dagDagArtifactGuardObject(value, at);
  return {
    id: dagDagArtifactGuardString(row["id"], `${at}.id`),
    source: dagDagArtifactGuardString(row["source"], `${at}.source`),
    target: dagDagArtifactGuardString(row["target"], `${at}.target`),
  };
}

export function parseDagCamera(value: unknown, at = "$"): DagCamera {
  const row = dagDagArtifactGuardObject(value, at);
  return {
    x: dagDagArtifactGuardNumber(row["x"], `${at}.x`),
    y: dagDagArtifactGuardNumber(row["y"], `${at}.y`),
    zoom: dagDagArtifactGuardNumber(row["zoom"], `${at}.zoom`),
  };
}
