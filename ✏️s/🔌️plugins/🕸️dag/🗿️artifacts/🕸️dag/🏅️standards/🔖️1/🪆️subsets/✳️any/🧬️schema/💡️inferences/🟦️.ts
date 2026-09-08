/** 💡️ Dag inference schema — topology (topological order + depth + cycle-freedom) over nodes/edges. */

export interface DagTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface DagInference {
  /** @derived */
  topology: DagTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class dagDagInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const dagDagInferenceGuardReject = (at: string, why: string): never => {
  throw new dagDagInferenceGuardRefusal(at, why);
};

type dagDagInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type dagDagInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type dagDagInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const dagDagInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : dagDagInferenceGuardReject(at, "value is not an object");
export const dagDagInferenceGuardArray = (value: unknown, at: string, bounds: dagDagInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return dagDagInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) dagDagInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) dagDagInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const dagDagInferenceGuardString = (value: unknown, at: string, bounds: dagDagInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return dagDagInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) dagDagInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) dagDagInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) dagDagInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const dagDagInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : dagDagInferenceGuardReject(at, "value is not a boolean"));
export const dagDagInferenceGuardNumber = (value: unknown, at: string, bounds: dagDagInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return dagDagInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) dagDagInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) dagDagInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const dagDagInferenceGuardInteger = (value: unknown, at: string, bounds: dagDagInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? dagDagInferenceGuardNumber(value, at, bounds) : dagDagInferenceGuardReject(at, "value is not an integer");
export const dagDagInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : dagDagInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const dagDagInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : dagDagInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDagInference(value: unknown, at = "$"): DagInference {
  const row = dagDagInferenceGuardObject(value, at);
  return {
    topology: parseDagTopology(row["topology"], `${at}.topology`),
  };
}

export function parseDagTopology(value: unknown, at = "$"): DagTopology {
  const row = dagDagInferenceGuardObject(value, at);
  return {
    topoOrder: dagDagInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => dagDagInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: dagDagInferenceGuardObject(row["depth"], `${at}.depth`),
    cycleFree: dagDagInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    nodeCount: dagDagInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`, {"minimum": 0}),
  };
}
