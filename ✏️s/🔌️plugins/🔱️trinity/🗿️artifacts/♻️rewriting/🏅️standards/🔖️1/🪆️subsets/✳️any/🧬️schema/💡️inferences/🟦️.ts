/** 💡️ Rewriting inference schema — bounds (bounding box + node count) over `ruleLayout`. */

export interface RewritingBoundingBox {
  minX: number;
  minY: number;
  maxX: number;
  maxY: number;
}

export interface RewritingBounds {
  boundingBox: RewritingBoundingBox;
  nodeCount: number;
}

export interface RewritingInference {
  /** @derived */
  bounds: RewritingBounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class trinityRewritingInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const trinityRewritingInferenceGuardReject = (at: string, why: string): never => {
  throw new trinityRewritingInferenceGuardRefusal(at, why);
};

type trinityRewritingInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type trinityRewritingInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type trinityRewritingInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const trinityRewritingInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : trinityRewritingInferenceGuardReject(at, "value is not an object");
export const trinityRewritingInferenceGuardArray = (value: unknown, at: string, bounds: trinityRewritingInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return trinityRewritingInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) trinityRewritingInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) trinityRewritingInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const trinityRewritingInferenceGuardString = (value: unknown, at: string, bounds: trinityRewritingInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return trinityRewritingInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) trinityRewritingInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) trinityRewritingInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) trinityRewritingInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const trinityRewritingInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : trinityRewritingInferenceGuardReject(at, "value is not a boolean"));
export const trinityRewritingInferenceGuardNumber = (value: unknown, at: string, bounds: trinityRewritingInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return trinityRewritingInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) trinityRewritingInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) trinityRewritingInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const trinityRewritingInferenceGuardInteger = (value: unknown, at: string, bounds: trinityRewritingInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? trinityRewritingInferenceGuardNumber(value, at, bounds) : trinityRewritingInferenceGuardReject(at, "value is not an integer");
export const trinityRewritingInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : trinityRewritingInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const trinityRewritingInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : trinityRewritingInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRewritingInference(value: unknown, at = "$"): RewritingInference {
  const row = trinityRewritingInferenceGuardObject(value, at);
  return {
    bounds: parseRewritingBounds(row["bounds"], `${at}.bounds`),
  };
}

export function parseRewritingBoundingBox(value: unknown, at = "$"): RewritingBoundingBox {
  const row = trinityRewritingInferenceGuardObject(value, at);
  return {
    minX: trinityRewritingInferenceGuardNumber(row["minX"], `${at}.minX`),
    minY: trinityRewritingInferenceGuardNumber(row["minY"], `${at}.minY`),
    maxX: trinityRewritingInferenceGuardNumber(row["maxX"], `${at}.maxX`),
    maxY: trinityRewritingInferenceGuardNumber(row["maxY"], `${at}.maxY`),
  };
}

export function parseRewritingBounds(value: unknown, at = "$"): RewritingBounds {
  const row = trinityRewritingInferenceGuardObject(value, at);
  return {
    boundingBox: parseRewritingBoundingBox(row["boundingBox"], `${at}.boundingBox`),
    nodeCount: trinityRewritingInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`, {"minimum": 0}),
  };
}
