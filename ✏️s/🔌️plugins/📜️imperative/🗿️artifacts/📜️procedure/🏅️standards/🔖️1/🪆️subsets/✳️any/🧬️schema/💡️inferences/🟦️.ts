/** 💡️ Imperative inference schema — topology (depth-first order + nesting depth + cycle-freedom) over the Path/Step tree. */

export interface ProcedureTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface ProcedureInference {
  /** @derived */
  topology: ProcedureTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class imperativeImperativeInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const imperativeImperativeInferenceGuardReject = (at: string, why: string): never => {
  throw new imperativeImperativeInferenceGuardRefusal(at, why);
};

type imperativeImperativeInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type imperativeImperativeInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type imperativeImperativeInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const imperativeImperativeInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : imperativeImperativeInferenceGuardReject(at, "value is not an object");
export const imperativeImperativeInferenceGuardArray = (value: unknown, at: string, bounds: imperativeImperativeInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return imperativeImperativeInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) imperativeImperativeInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) imperativeImperativeInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const imperativeImperativeInferenceGuardString = (value: unknown, at: string, bounds: imperativeImperativeInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return imperativeImperativeInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) imperativeImperativeInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) imperativeImperativeInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) imperativeImperativeInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const imperativeImperativeInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : imperativeImperativeInferenceGuardReject(at, "value is not a boolean"));
export const imperativeImperativeInferenceGuardNumber = (value: unknown, at: string, bounds: imperativeImperativeInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return imperativeImperativeInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) imperativeImperativeInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) imperativeImperativeInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const imperativeImperativeInferenceGuardInteger = (value: unknown, at: string, bounds: imperativeImperativeInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? imperativeImperativeInferenceGuardNumber(value, at, bounds) : imperativeImperativeInferenceGuardReject(at, "value is not an integer");
export const imperativeImperativeInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : imperativeImperativeInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const imperativeImperativeInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : imperativeImperativeInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProcedureInference(value: unknown, at = "$"): ProcedureInference {
  const row = imperativeImperativeInferenceGuardObject(value, at);
  return {
    topology: parseProcedureTopology(row["topology"], `${at}.topology`),
  };
}

export function parseProcedureTopology(value: unknown, at = "$"): ProcedureTopology {
  const row = imperativeImperativeInferenceGuardObject(value, at);
  return {
    topoOrder: imperativeImperativeInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => imperativeImperativeInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: imperativeImperativeInferenceGuardObject(row["depth"], `${at}.depth`),
    cycleFree: imperativeImperativeInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    nodeCount: imperativeImperativeInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`, {"minimum": 0}),
  };
}
