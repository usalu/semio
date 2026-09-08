/** 💡️ Semio graph inference schema — node/edge topological order (Kahn's algorithm). */

export interface SemioGraphTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface SemioGraphInference {
  /** @derived */
  topology: SemioGraphTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1GraphInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1GraphInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1GraphInferenceGuardRefusal(at, why);
};

type stdioSemioV1GraphInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1GraphInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1GraphInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1GraphInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1GraphInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1GraphInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1GraphInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1GraphInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1GraphInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1GraphInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1GraphInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1GraphInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1GraphInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1GraphInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1GraphInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1GraphInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1GraphInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1GraphInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1GraphInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1GraphInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1GraphInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1GraphInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1GraphInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1GraphInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1GraphInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1GraphInferenceGuardNumber(value, at, bounds) : stdioSemioV1GraphInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1GraphInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1GraphInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1GraphInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1GraphInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioGraphInference(value: unknown, at = "$"): SemioGraphInference {
  const row = stdioSemioV1GraphInferenceGuardObject(value, at);
  return {
    topology: parseSemioGraphTopology(row["topology"], `${at}.topology`),
  };
}

export function parseSemioGraphTopology(value: unknown, at = "$"): SemioGraphTopology {
  const row = stdioSemioV1GraphInferenceGuardObject(value, at);
  return {
    topoOrder: stdioSemioV1GraphInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => stdioSemioV1GraphInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: stdioSemioV1GraphInferenceGuardObject(row["depth"], `${at}.depth`),
    cycleFree: stdioSemioV1GraphInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    nodeCount: stdioSemioV1GraphInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`),
  };
}
