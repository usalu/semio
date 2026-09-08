/** 💡️ Semio flow inference schema — node/edge topological order (Kahn's algorithm). */

export interface SemioFlowTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface SemioFlowInference {
  /** @derived */
  topology: SemioFlowTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1FlowInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1FlowInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1FlowInferenceGuardRefusal(at, why);
};

type stdioSemioV1FlowInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1FlowInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1FlowInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1FlowInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1FlowInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1FlowInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1FlowInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1FlowInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1FlowInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1FlowInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1FlowInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1FlowInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1FlowInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1FlowInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1FlowInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1FlowInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1FlowInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1FlowInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1FlowInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1FlowInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1FlowInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1FlowInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1FlowInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1FlowInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1FlowInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1FlowInferenceGuardNumber(value, at, bounds) : stdioSemioV1FlowInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1FlowInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1FlowInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1FlowInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1FlowInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioFlowInference(value: unknown, at = "$"): SemioFlowInference {
  const row = stdioSemioV1FlowInferenceGuardObject(value, at);
  return {
    topology: parseSemioFlowTopology(row["topology"], `${at}.topology`),
  };
}

export function parseSemioFlowTopology(value: unknown, at = "$"): SemioFlowTopology {
  const row = stdioSemioV1FlowInferenceGuardObject(value, at);
  return {
    topoOrder: stdioSemioV1FlowInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => stdioSemioV1FlowInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: stdioSemioV1FlowInferenceGuardObject(row["depth"], `${at}.depth`),
    cycleFree: stdioSemioV1FlowInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    nodeCount: stdioSemioV1FlowInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`),
  };
}
