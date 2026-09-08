/** 💡️ Jack inference schema — topology (topological order, depth, cycle-freedom) over nodes/edges,
 *  and flat-position (each node's flattened `(u, v)` position, BFS-walked from root_node_id). */

export interface JackTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface JackFlatPositionUv {
  u: number;
  v: number;
}

export interface JackFlatPosition {
  positions: Record<string, JackFlatPositionUv>;
}

export interface JackInference {
  /** @derived */
  topology: JackTopology;
  /** @derived */
  flatPosition: JackFlatPosition;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class trinityJackInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const trinityJackInferenceGuardReject = (at: string, why: string): never => {
  throw new trinityJackInferenceGuardRefusal(at, why);
};

type trinityJackInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type trinityJackInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type trinityJackInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const trinityJackInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : trinityJackInferenceGuardReject(at, "value is not an object");
export const trinityJackInferenceGuardArray = (value: unknown, at: string, bounds: trinityJackInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return trinityJackInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) trinityJackInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) trinityJackInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const trinityJackInferenceGuardString = (value: unknown, at: string, bounds: trinityJackInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return trinityJackInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) trinityJackInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) trinityJackInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) trinityJackInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const trinityJackInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : trinityJackInferenceGuardReject(at, "value is not a boolean"));
export const trinityJackInferenceGuardNumber = (value: unknown, at: string, bounds: trinityJackInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return trinityJackInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) trinityJackInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) trinityJackInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const trinityJackInferenceGuardInteger = (value: unknown, at: string, bounds: trinityJackInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? trinityJackInferenceGuardNumber(value, at, bounds) : trinityJackInferenceGuardReject(at, "value is not an integer");
export const trinityJackInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : trinityJackInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const trinityJackInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : trinityJackInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJackInference(value: unknown, at = "$"): JackInference {
  const row = trinityJackInferenceGuardObject(value, at);
  return {
    topology: parseJackTopology(row["topology"], `${at}.topology`),
    flatPosition: parseJackFlatPosition(row["flatPosition"], `${at}.flatPosition`),
  };
}

export function parseJackTopology(value: unknown, at = "$"): JackTopology {
  const row = trinityJackInferenceGuardObject(value, at);
  return {
    topoOrder: trinityJackInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => trinityJackInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: trinityJackInferenceGuardObject(row["depth"], `${at}.depth`),
    cycleFree: trinityJackInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    nodeCount: trinityJackInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`, {"minimum": 0}),
  };
}

export function parseJackFlatPosition(value: unknown, at = "$"): JackFlatPosition {
  const row = trinityJackInferenceGuardObject(value, at);
  return {
    positions: trinityJackInferenceGuardObject(row["positions"], `${at}.positions`),
  };
}

export function parseJackFlatPositionUv(value: unknown, at = "$"): JackFlatPositionUv {
  const row = trinityJackInferenceGuardObject(value, at);
  return {
    u: trinityJackInferenceGuardNumber(row["u"], `${at}.u`),
    v: trinityJackInferenceGuardNumber(row["v"], `${at}.v`),
  };
}
