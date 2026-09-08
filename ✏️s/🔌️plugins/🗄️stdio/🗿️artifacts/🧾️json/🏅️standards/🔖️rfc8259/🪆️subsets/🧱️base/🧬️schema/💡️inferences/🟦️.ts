/** 💡️ Json inference schema — document outline (node count, max depth, root kind). */

export interface JsonOutline {
  nodeCount: number;
  maxDepth: number;
  rootKind: string;
}

export interface JsonInference {
  /** @derived */
  outline: JsonOutline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioJsonRfc8259BaseInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioJsonRfc8259BaseInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioJsonRfc8259BaseInferenceGuardRefusal(at, why);
};

type stdioJsonRfc8259BaseInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioJsonRfc8259BaseInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioJsonRfc8259BaseInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioJsonRfc8259BaseInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioJsonRfc8259BaseInferenceGuardReject(at, "value is not an object");
export const stdioJsonRfc8259BaseInferenceGuardArray = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioJsonRfc8259BaseInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioJsonRfc8259BaseInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioJsonRfc8259BaseInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioJsonRfc8259BaseInferenceGuardString = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioJsonRfc8259BaseInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioJsonRfc8259BaseInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioJsonRfc8259BaseInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioJsonRfc8259BaseInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioJsonRfc8259BaseInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioJsonRfc8259BaseInferenceGuardReject(at, "value is not a boolean"));
export const stdioJsonRfc8259BaseInferenceGuardNumber = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioJsonRfc8259BaseInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioJsonRfc8259BaseInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioJsonRfc8259BaseInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioJsonRfc8259BaseInferenceGuardInteger = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioJsonRfc8259BaseInferenceGuardNumber(value, at, bounds) : stdioJsonRfc8259BaseInferenceGuardReject(at, "value is not an integer");
export const stdioJsonRfc8259BaseInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioJsonRfc8259BaseInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioJsonRfc8259BaseInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioJsonRfc8259BaseInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJsonInference(value: unknown, at = "$"): JsonInference {
  const row = stdioJsonRfc8259BaseInferenceGuardObject(value, at);
  return {
    outline: parseJsonOutline(row["outline"], `${at}.outline`),
  };
}

export function parseJsonOutline(value: unknown, at = "$"): JsonOutline {
  const row = stdioJsonRfc8259BaseInferenceGuardObject(value, at);
  return {
    nodeCount: stdioJsonRfc8259BaseInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`),
    maxDepth: stdioJsonRfc8259BaseInferenceGuardInteger(row["maxDepth"], `${at}.maxDepth`),
    rootKind: stdioJsonRfc8259BaseInferenceGuardString(row["rootKind"], `${at}.rootKind`),
  };
}
